using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;
    
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
                    DontDestroyOnLoad(instanceObject);
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
        private Registry<IIdentifier, IIdentifiable> instanceRegistry;
        #endregion

        #region Methods
        public void RegisterInstance(CSharpInstance instance)
        {
            instanceRegistry.Register(instance.InstanceIdentifier, instance);
        }

        public void UnregisterInstance(CSharpInstance instance)
        {
            instanceRegistry.Unregister(instance.InstanceIdentifier);
        }

        public CSharpInstance GetInstance(InstanceIdentifier instanceIdentifier)
        {
            return (CSharpInstance)instanceRegistry.Get(instanceIdentifier);
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

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Registry Registration
            RegistryManager registryManager = RegistryManager.Instance;
            instanceRegistry = new Registry<IIdentifier, IIdentifiable>("LooCast:InstanceIdentifier_LooCast:CSharpInstance");
            registryManager.RegisterRegistry(instanceRegistry);
            #endregion
        }
        #endregion
    }
}