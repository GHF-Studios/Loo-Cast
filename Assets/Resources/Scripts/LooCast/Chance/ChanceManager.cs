using System;
using UnityEngine;

namespace LooCast.Chance
{
    public class ChanceManager : ModuleManager
    {
        #region Static Properties
        public static ChanceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[ChanceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<ChanceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static ChanceManager instance;
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
            looCastNamespace = new Namespace("Chance", rootNamespace);
            looCastType = new Type(typeof(ChanceManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);
            
            Type chanceType = new Type(typeof(Chance), looCastNamespace);
            Type seedType = new Type(typeof(Seed<IComparable>), looCastNamespace);

            typeManager.RegisterType(chanceType);
            typeManager.RegisterType(seedType);
            #endregion
        }
        #endregion
    }
}