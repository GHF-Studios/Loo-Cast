using System;
using UnityEngine;

namespace LooCast
{
    public class InstanceManager : InternalManager
    {
        #region Static Properties
        public static InstanceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[InstanceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<InstanceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static InstanceManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        #endregion

        #region Methods
        public void RegisterInstance(Instance instance)
        {
            Registry<IIdentifier, IIdentifiable> instanceRegistry = RegistryManager.Instance.GetRegistry("LooCast:InstanceIdentifier_LooCast:Instance");
            instanceRegistry.Register(instance.InstanceIdentifier, instance);
        }

        public void UnregisterInstance(Instance instance)
        {
            Registry<IIdentifier, IIdentifiable> instanceRegistry = RegistryManager.Instance.GetRegistry("LooCast:InstanceIdentifier_LooCast:Instance");
            instanceRegistry.Unregister(instance.InstanceIdentifier);
        }

        public Instance GetInstance(InstanceIdentifier instanceIdentifier)
        {
            Registry<IIdentifier, IIdentifiable> instanceRegistry = RegistryManager.Instance.GetRegistry("LooCast:InstanceIdentifier_LooCast:Instance");
            return (Instance)instanceRegistry.Get(instanceIdentifier);
        }
        #endregion

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;

            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(InstanceManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            typeManager.RegisterType(looCastType);
            RegisterInstance(looCastInstance);
            #endregion
        }
        #endregion
    }
}