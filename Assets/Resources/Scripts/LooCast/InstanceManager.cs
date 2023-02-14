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
        public override Namespace LooCastNamespace => looCastNamespace;
        public override Type LooCastType => looCastType;
        public override Instance LooCastInstance => looCastInstance;
        #endregion

        #region Fields
        private Namespace looCastNamespace;
        private Type looCastType;
        private Instance looCastInstance;
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

            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;

            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(InstanceManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            typeManager.RegisterType(looCastType);
            RegisterInstance(looCastInstance);
        }
        #endregion
    }
}