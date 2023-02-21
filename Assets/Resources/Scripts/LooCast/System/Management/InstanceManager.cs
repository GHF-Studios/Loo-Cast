﻿using System;
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
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[InstanceManager]");
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
        private Registry<IInstanceIdentifier, IInstanceIdentifiable> instanceRegistry;
        #endregion

        #region Methods
        public void RegisterInstance(IInstance instance)
        {
            instanceRegistry.Register(instance.InstanceIdentifier, instance);
        }

        public void UnregisterInstance(IInstance instance)
        {
            instanceRegistry.Unregister(instance.InstanceIdentifier);
        }

        public IInstance GetInstance(IInstanceIdentifier instanceIdentifier)
        {
            return (IInstance)instanceRegistry.Get(instanceIdentifier);
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
            looCastUnityInstance = new Instance(this, looCastType);

            typeManager.RegisterType(looCastType);
            RegisterInstance(looCastUnityInstance);
            #endregion
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Registry Registration
            RegistryManager registryManager = RegistryManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            IType keyType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:IInstanceIdentifier"));
            IType valueType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:IInstanceIdentifiable"));
            instanceRegistry = new Registry<IInstanceIdentifier, IInstanceIdentifiable>(keyType, valueType);
            registryManager.RegisterRegistry(instanceRegistry);
            #endregion
        }
        #endregion
    }
}