using System;
using UnityEngine;

namespace LooCast.Noise
{
    public class NoiseManager : ModuleManager
    {
        #region Static Properties
        public static NoiseManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[NoiseManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<NoiseManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static NoiseManager instance;
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
            looCastNamespace = new Namespace("Data", rootNamespace);
            looCastType = new Type(typeof(DataManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type dataType1 = new Type(typeof(DataType1), looCastNamespace);
            Type dataType2 = new Type(typeof(DataType2), looCastNamespace);

            typeManager.RegisterType(dataType1);
            typeManager.RegisterType(dataType2);
            #endregion
        }
        #endregion
    }
}