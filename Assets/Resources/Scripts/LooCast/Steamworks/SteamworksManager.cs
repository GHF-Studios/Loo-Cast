// The SteamManager is designed to work with Steamworks.NET
// This file is released into the public domain.
// Where that dedication is not recognized you are granted a perpetual,
// irrevocable license to copy and modify this file as you see fit.
//
// Version: 1.0.12

#if !(UNITY_STANDALONE_WIN || UNITY_STANDALONE_LINUX || UNITY_STANDALONE_OSX || STEAMWORKS_WIN || STEAMWORKS_LIN_OSX)
	#define DISABLESTEAMWORKS
#endif

#if !DISABLESTEAMWORKS
using Steamworks;
#endif

using UnityEngine;
using CSSystem = System;

namespace LooCast.Steamworks
{
    using global::LooCast.System;
    using global::LooCast.System.Managers;
	using global::LooCast.System.MetaData;

	//
	// The SteamManager provides a base implementation of Steamworks.NET on which you can build upon.
	// It handles the basics of starting up and shutting down the SteamAPI for use.
	//
	[DisallowMultipleComponent]
	public sealed class SteamworksManager : ModuleManager<SteamworksManager, SteamworksManagerMetaData>
	{
#if !DISABLESTEAMWORKS
		
        #region Static Properties
        public static bool Initialized
        {
            get
            {
                return Instance.initialized;
            }
        }
        #endregion

        #region Static Fields
        private static SteamworksManager instance;
        private static bool EverInitialized = false;
        #endregion

        #region Fields
        private bool initialized = false;
        private SteamAPIWarningMessageHook_t m_SteamAPIWarningMessageHook;
        #endregion

        #region Static Methods
#if UNITY_2019_3_OR_NEWER
        // In case of disabled Domain Reload, reset static members before entering Play Mode.
        [RuntimeInitializeOnLoadMethod(RuntimeInitializeLoadType.SubsystemRegistration)]
        private static void InitOnPlayMode()
        {
            EverInitialized = false;
            instance = null;
        }
#endif
		
        [AOT.MonoPInvokeCallback(typeof(SteamAPIWarningMessageHook_t))]
        private static void SteamAPIDebugTextHook(int nSeverity, CSSystem.Text.StringBuilder pchDebugText)
        {
            Debug.LogWarning(pchDebugText);
        }
        #endregion

        #region Unity Callbacks
        private void Awake()
		{
			// Only one gameObjectInstance of SteamManager at a time!
			if (instance != null)
			{
				Destroy(gameObject);
				return;
			}
			instance = this;

			if (EverInitialized)
			{
				// This is almost always an error.
				// The most common case where this happens is when SteamManager gets destroyed because of Application.Quit(),
				// and then some Steamworks code in some other OnDestroy gets called afterwards, creating a new SteamManager.
				// You should never call Steamworks functions in OnDestroy, always prefer OnDisable if possible.
				throw new global::System.Exception("Tried to Initialize the SteamAPI twice in one session!");
			}

			// We want our SteamManager CSharpInstance to persist across scenes.
			DontDestroyOnLoad(gameObject);

			if (!Packsize.Test())
			{
				Debug.LogError("[SteamManager] Packsize Test returned false, the wrong version of Steamworks.NET is being run in this platform.", this);
			}

			if (!DllCheck.Test())
			{
				Debug.LogError("[SteamManager] DllCheck Test returned false, One or more of the Steamworks binaries seems to be the wrong version.", this);
			}

			try
			{
				// If Steam is not running or the game wasn't started through Steam, SteamAPI_RestartAppIfNecessary starts the
				// Steam client and also launches this game again if the User owns it. This can act as a rudimentary form of DRM.

				// Once you get a Steam AppID assigned by Valve, you need to replace AppId_t.Invalid with it and
				// remove steam_appid.txt from the game depot. eg: "(AppId_t)480" or "new AppId_t(480)".
				// See the Valve documentation for more information: https://partner.steamgames.com/doc/sdk/api#initialization_and_shutdown
				if (SteamAPI.RestartAppIfNecessary((AppId_t)2122620))
				{
					Application.Quit();
					return;
				}
			}
			catch (global::System.DllNotFoundException e)
			{
				// We catch this exception here, as it will be the first occurrence of it.
				Debug.LogError("[SteamManager] Could not load [lib]steam_api.dll/so/dylib. It's likely not in the correct location. Refer to the README for more details.\n" + e, this);

				Application.Quit();
				return;
			}

			// Initializes the Steamworks API.
			// If this returns false then this indicates one of the following conditions:
			// [*] The Steam client isn't running. A running Steam client is required to provide implementations of the various Steamworks interfaces.
			// [*] The Steam client couldn't determine the App ID of game. If you're running your application from the executable or debugger directly then you must have a [code-inline]steam_appid.txt[/code-inline] in your game directory next to the executable, with your app ID in it and nothing else. Steam will look for this file in the current working directory. If you are running your executable from a different directory you may need to relocate the [code-inline]steam_appid.txt[/code-inline] file.
			// [*] Your application is not running under the same OS user context as the Steam client, such as a different user or administration access level.
			// [*] Ensure that you own a license for the App ID on the currently active Steam account. Your game must show up in your Steam library.
			// [*] Your App ID is not completely set up, i.e. in Release State: Unavailable, or it's missing default packages.
			// Valve's documentation for this is located here:
			// https://partner.steamgames.com/doc/sdk/api#initialization_and_shutdown
			initialized = SteamAPI.Init();
			if (!initialized)
			{
				Debug.LogError("[SteamManager] SteamAPI_Init() failed. Refer to Valve's documentation or the comment above this line for more information.", this);

				return;
			}

			Debug.Log($"[SteamManager] Initialized.");

			EverInitialized = true;
		}

        // This should only ever get called on first load and after an Assembly reload, You should never Disable the Steamworks Manager yourself.
        private void OnEnable()
		{
			if (instance == null)
			{
				instance = this;
			}

			if (!initialized)
			{
				return;
			}

			if (m_SteamAPIWarningMessageHook == null)
			{
				// Set up our callback to receive warning messages from Steam.
				// You must launch with "-debug_steamapi" in the launch args to receive warnings.
				m_SteamAPIWarningMessageHook = new SteamAPIWarningMessageHook_t(SteamAPIDebugTextHook);
				SteamClient.SetWarningMessageHook(m_SteamAPIWarningMessageHook);
			}
		}

        // OnApplicationQuit gets called too early to shutdown the SteamAPI.
        // Because the SteamManager should be persistent and never disabled or destroyed we can shutdown the SteamAPI here.
        // Thus it is not recommended to perform any Steamworks work in other OnDestroy functions as the order of execution can not be garenteed upon Shutdown. Prefer OnDisable().
        private void OnDestroy()
		{
			if (instance != this)
			{
				return;
			}

			instance = null;

			if (!initialized)
			{
				return;
			}

			SteamAPI.Shutdown();
		}

		private void Update()
		{
			if (!initialized)
			{
				return;
			}

			// Run Steam client callbacks
			SteamAPI.RunCallbacks();
        }
        #endregion

        #region Overrides
        protected override IManager GetParentManager()
        {
			return global::LooCast.Core.CoreManager.Instance;
        }
        #endregion

#else
		
		public static bool Initialized
		{
			get
			{
				return false;
			}
		}
		
#endif
    }
}