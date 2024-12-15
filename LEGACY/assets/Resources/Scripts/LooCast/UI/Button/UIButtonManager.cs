using System;
using UnityEngine;

namespace LooCast.UI.Button
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class UIButtonManager : SubModuleManager
    {
        #region Static Properties
        public static UIButtonManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[UIButtonManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UIButtonManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIButtonManager instance;
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
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast.UI");
            looCastNamespace = new Namespace("Button", rootNamespace);
            looCastType = new Type(typeof(UIButtonManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type attributeSetButtonType = new Type(typeof(AttributeSetButton), looCastNamespace);
            Type buttonType = new Type(typeof(Button), looCastNamespace);
            Type createGameButtonType = new Type(typeof(CreateGameButton), looCastNamespace);
            Type hardresetButtonType = new Type(typeof(HardresetButton), looCastNamespace);
            Type loadGameButtonType = new Type(typeof(LoadGameButton), looCastNamespace);
            Type loadMainMenuButtonType = new Type(typeof(LoadMainMenuButton), looCastNamespace);
            Type missionButtonType = new Type(typeof(MissionButton), looCastNamespace);
            Type statSetButtonType = new Type(typeof(StatSetButton), looCastNamespace);
            Type tabButtonType = new Type(typeof(TabButton), looCastNamespace);

            typeManager.RegisterType(attributeSetButtonType);
            typeManager.RegisterType(buttonType);
            typeManager.RegisterType(createGameButtonType);
            typeManager.RegisterType(hardresetButtonType);
            typeManager.RegisterType(loadGameButtonType);
            typeManager.RegisterType(loadMainMenuButtonType);
            typeManager.RegisterType(missionButtonType);
            typeManager.RegisterType(statSetButtonType);
            typeManager.RegisterType(tabButtonType);
            #endregion
        }
        #endregion
    }
}