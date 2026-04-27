using System;
using UnityEngine;

namespace LooCast.Chance
{
    using LooCast.System;
    using LooCast.System.Managers;

    public class ChanceManager : ModuleManager
    {
        #region Static Properties
        public static ChanceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[ChanceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
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
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            INamespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Chance", rootNamespace);
            looCastType = new Type(typeof(ChanceManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            
            Type chanceType = new Type(typeof(Chance), looCastNamespace);
            Type seedType = new Type(typeof(Seed<IComparable>), looCastNamespace);

            typeManager.RegisterType(chanceType);
            typeManager.RegisterType(seedType);
            #endregion
        }
        #endregion
    }
}