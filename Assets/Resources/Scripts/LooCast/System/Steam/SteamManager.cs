using UnityEngine;
using System.Collections;
using Steamworks;
using CSSystem = System;
using System.Text;

namespace LooCast.System.Steam
{
    using LooCast.System;
    using LooCast.System.ECS;

    public sealed class SteamManager : ModuleManager
    {
        #region Static Properties
        public static SteamManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<SteamManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static bool everInitialized = false;
        private static SteamManager instance = null;
        #endregion

        #region Properties
        public static bool Initialized
        {
            get
            {
                return Instance.initialized;
            }
        }
        #endregion

        #region Fields
        private bool initialized = false;
        private SteamAPIWarningMessageHook_t steamAPIWarningMessageHook;
        #endregion

        #region Constructors
        public SteamManager() : base()
        {
            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedSteamManagerEntityTypeName = typeof(SteamManager).AssemblyQualifiedName;
                string assemblyQualifiedSteamManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedSteamManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSteamManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSteamManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSteamManagerEntityDataTypeName;
                instanceMetaData.EntityID = new CSSystem.Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSteamManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSteamManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSteamManagerEntityDataTypeName;
                instanceData.ManagerName = "SteamManager";
                instanceData.ManagerParent = SystemManager.Instance;

                SetEntityMetaData(instanceMetaData);
                SetEntityData(instanceData);

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component's data here

                // Register pre-included components here

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPostSetup();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                // Pre-Initialize pre-included components here
            });

            RegisterInitializationAction(() =>
            {
                // Initialize pre-included components here
            });

            RegisterPostInitializationAction(() =>
            {
                // Post-Initialize pre-included components here
            });
        }
        #endregion

        #region Overrides
        protected override void OnUnityAwake()
        {
            // Only one instance of SteamworksManagerUnityComponent at a time!
            if (instance != null)
            {
                throw new CSSystem.Exception("Only one instance of SteamManager can exist at a time!");
            }
            instance = this;

            if (everInitialized)
            {
                // This is almost always an error.
                // The most common case where this happens is when SteamworksManagerUnityComponent gets destroyed because of Application.Quit(),
                // and then some Steamworks code in some other OnDestroy gets called afterwards, creating a new SteamworksManagerUnityComponent.
                // You should never call Steamworks functions in OnDestroy, always prefer OnDisable if possible.
                throw new CSSystem.Exception("Tried to Initialize the SteamAPI twice in one session!");
            }

            if (!Packsize.Test())
            {
                Debug.LogError("[Steamworks.NET] Packsize Test returned false, the wrong version of Steamworks.NET is being run in this platform.", UnityBridge.RootGameObject);
            }

            if (!DllCheck.Test())
            {
                Debug.LogError("[Steamworks.NET] DllCheck Test returned false, One or more of the Steamworks binaries seems to be the wrong version.", UnityBridge.RootGameObject);
            }

            try
            {
                // If Steam is not running or the game wasn't started through Steam, SteamAPI_RestartAppIfNecessary starts the
                // Steam client and also launches this game again if the User owns it. This can act as a rudimentary form of DRM.
                if (SteamAPI.RestartAppIfNecessary((AppId_t)2122620))
                {
                    Application.Quit();
                    return;
                }
            }
            catch (CSSystem.DllNotFoundException e)
            {
                // We catch this exception here, as it will be the first occurrence of it.
                Debug.LogError("[Steamworks.NET] Could not load [lib]steam_api.dll/so/dylib. It's likely not in the correct location. Refer to the README for more details.\n" + e, UnityBridge.RootGameObject);

                Application.Quit();
                return;
            }

            // Initializes the Steamworks API.
            // If this returns false then this indicates one of the following conditions:
            // [*] The Steam client isn't running. A running Steam client is required to provide implementations of the various Steamworks interfaces.
            // [*] The Steam client couldn't determine the App ID of game. If you're running your application from the executable or debugger directly then you must have a [code-inline]steam_appid.txt[/code-inline] in your game directory next to the executable, with your app ID in it and nothing else. Steam will look for this hierarchyFile in the current working directory. If you are running your executable from a different directory you may need to relocate the [code-inline]steam_appid.txt[/code-inline] hierarchyFile.
            // [*] Your application is not running under the same OS user context as the Steam client, such as a different user or administration access level.
            // [*] Ensure that you own a license for the App ID on the currently active Steam account. Your game must show up in your Steam library.
            // [*] Your App ID is not completely set up, i.e. in Release State: Unavailable, or it's missing default packages.
            // Valve's documentation for this is located here:
            // https://partner.steamgames.com/doc/sdk/api#initialization_and_shutdown
            initialized = SteamAPI.Init();
            if (!initialized)
            {
                Debug.LogError("[Steamworks.NET] SteamAPI_Init() failed. Refer to Valve's documentation or the comment above this line for more information.", UnityBridge.RootGameObject);

                return;
            }

            everInitialized = true;
        }

        // This should only ever get called on first load and after an Assembly reload, You should never Disable the Steamworks Manager yourself.
        protected override void OnUnityEnable()
        {
            if (instance == null)
            {
                instance = this;
            }

            if (!initialized)
            {
                return;
            }

            if (steamAPIWarningMessageHook == null)
            {
                // Set up our callback to receive warning messages from Steam.
                // You must launch with "-debug_steamapi" in the launch args to receive warnings.
                steamAPIWarningMessageHook = new SteamAPIWarningMessageHook_t(SteamAPIDebugTextHook);
                SteamClient.SetWarningMessageHook(steamAPIWarningMessageHook);
            }
        }
        
        // OnApplicationQuit gets called too early to shutdown the SteamAPI.
        // Because the SteamworksManagerUnityComponent should be persistent and never disabled or destroyed we can shutdown the SteamAPI here.
        // Thus it is not recommended to perform any Steamworks work in other OnDestroy functions as the order of execution can not be guarenteed upon Shutdown. Prefer OnDisable().
        protected override void OnUnityDestroy()
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

        protected override void OnUnityUpdate()
        {
            if (!initialized)
            {
                return;
            }

            // Run Steam client callbacks
            SteamAPI.RunCallbacks();
        }
        #endregion

        #region Static Methods
        [AOT.MonoPInvokeCallback(typeof(SteamAPIWarningMessageHook_t))]
        private static void SteamAPIDebugTextHook(int severity, StringBuilder pchDebugText)
        {
            Debug.LogWarning(pchDebugText);
        }
        #endregion
    }
}
