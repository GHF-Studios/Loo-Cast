using System;
using UnityEngine;

namespace LooCast.UI.Value
{
    using LooCast.System;
    using LooCast.System.Management;

    public class UIValueManager : SubModuleManager
    {
        #region Static Properties
        public static UIValueManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[UIValueManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = UIManager.Instance.transform;
                    return instanceObject.AddComponent<UIValueManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIValueManager instance;
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
            looCastNamespace = new Namespace("Value", rootNamespace);
            looCastType = new Type(typeof(UIValueManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type coinsValueType = new Type(typeof(CoinsValue), looCastNamespace);
            Type experienceLevelValueType = new Type(typeof(ExperienceLevelValue), looCastNamespace);
            Type statValueType = new Type(typeof(StatValue), looCastNamespace);
            Type tokensValueType = new Type(typeof(TokensValue), looCastNamespace);
            Type valueType = new Type(typeof(Value), looCastNamespace);

            typeManager.RegisterType(coinsValueType);
            typeManager.RegisterType(experienceLevelValueType);
            typeManager.RegisterType(statValueType);
            typeManager.RegisterType(tokensValueType);
            typeManager.RegisterType(valueType);
            #endregion
        }
        #endregion
    }
}