using System;
using UnityEngine;
using System.Collections.Generic;

namespace LooCast
{
    public class RegistryManager : InternalManager
    {
        #region Static Properties
        public static RegistryManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[RegistryManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    return instanceObject.AddComponent<RegistryManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static RegistryManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        
        private Dictionary<RegistryIdentifier, Registry<IIdentifier, IIdentifiable>> registries;
        #endregion

        #region Methods
        public void RegisterRegistry(Registry<IIdentifier, IIdentifiable> registry)
        {
            RegistryIdentifier registryIdentifier = registry.RegistryIdentifier;
            if (registries.ContainsKey(registryIdentifier))
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' already exists!");
            }

            registries.Add(registryIdentifier, registry);
        }

        public Registry<IIdentifier, IIdentifiable> GetRegistry(RegistryIdentifier registryIdentifier)
        {
            if (registries.ContainsKey(registryIdentifier))
            {
                return registries[registryIdentifier];
            }
            else
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' does not exist!");
            }
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            registries = new Dictionary<RegistryIdentifier, Registry<IIdentifier, IIdentifiable>>();
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;
            
            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new Type(typeof(RegistryManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);
            #endregion
        }
        #endregion
    }
}
