using System;
using UnityEngine;

namespace LooCast.UI.Level
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UILevelManager : SubModuleManager
    {
        #region Static Properties
        public static UILevelManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[UILevelManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UILevelManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UILevelManager instance;
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
            looCastNamespace = new Namespace("Level", rootNamespace);
            looCastType = new Type(typeof(UILevelManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type attributeLevelType = new Type(typeof(AttributeLevel), looCastNamespace);
            Type levelType = new Type(typeof(Level), looCastNamespace);
            Type statLevelType = new Type(typeof(StatLevel), looCastNamespace);

            typeManager.RegisterType(attributeLevelType);
            typeManager.RegisterType(levelType);
            typeManager.RegisterType(statLevelType);
            #endregion
        }
        #endregion
    }
}