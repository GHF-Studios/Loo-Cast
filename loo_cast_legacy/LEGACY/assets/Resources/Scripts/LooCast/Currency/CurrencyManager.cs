using System;
using UnityEngine;

namespace LooCast.Currency
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class CurrencyManager : ModuleManager
    {
        #region Static Properties
        public static CurrencyManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[CurrencyManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<CurrencyManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static CurrencyManager instance;
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

            INamespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Currency", rootNamespace);
            looCastType = new Type(typeof(CurrencyManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);

            Type coinsType = new Type(typeof(Coins), looCastNamespace);
            Type tokensType = new Type(typeof(Tokens), looCastNamespace);
            Type creditsType = new Type(typeof(Credits), looCastNamespace);

            typeManager.RegisterType(coinsType);
            typeManager.RegisterType(tokensType);
            typeManager.RegisterType(creditsType);
            #endregion
        }
        #endregion
    }
}