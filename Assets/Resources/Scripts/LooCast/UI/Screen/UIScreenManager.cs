using System;
using UnityEngine;

namespace LooCast.UI.Screen
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UIScreenManager : SubModuleManager
    {
        #region Static Properties
        public static UIScreenManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UIScreenManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UIScreenManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIScreenManager instance;
        #endregion

        #region Fields

        #endregion

        #region Methods
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.UI");
            looCastNamespace = new Namespace("Screen", rootNamespace);
            looCastType = new Type(typeof(UIScreenManager), looCastNamespace);
            looCastUnityInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);

            Type deathScreenType = new Type(typeof(DeathScreen), looCastNamespace);
            Type loadGameScreenType = new Type(typeof(LoadGameScreen), looCastNamespace);
            Type loadingScreenType = new Type(typeof(LoadingScreen), looCastNamespace);
            Type mainScreenType = new Type(typeof(MainScreen), looCastNamespace);
            Type newGameScreenType = new Type(typeof(NewGameScreen), looCastNamespace);
            Type pauseScreenType = new Type(typeof(PauseScreen), looCastNamespace);
            Type screenType = new Type(typeof(Screen), looCastNamespace);
            Type settingsScreenType = new Type(typeof(SettingsScreen), looCastNamespace);
            Type stationScreenType = new Type(typeof(StationScreen), looCastNamespace);
            Type statsScreenType = new Type(typeof(StatsScreen), looCastNamespace);

            typeManager.RegisterType(deathScreenType);
            typeManager.RegisterType(loadGameScreenType);
            typeManager.RegisterType(loadingScreenType);
            typeManager.RegisterType(mainScreenType);
            typeManager.RegisterType(newGameScreenType);
            typeManager.RegisterType(pauseScreenType);
            typeManager.RegisterType(screenType);
            typeManager.RegisterType(settingsScreenType);
            typeManager.RegisterType(stationScreenType);
            typeManager.RegisterType(statsScreenType);
            #endregion
        }
        #endregion
    }
}