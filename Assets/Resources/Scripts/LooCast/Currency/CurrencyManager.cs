using System;
using UnityEngine;

namespace LooCast.Currency
{
    public class CurrencyManager : ModuleManager
    {
        #region Static Properties
        public static CurrencyManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[CurrencyManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
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
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Currency", rootNamespace);
            looCastType = new Type(typeof(CurrencyManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

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