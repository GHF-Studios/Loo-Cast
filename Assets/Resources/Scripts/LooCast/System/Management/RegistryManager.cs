using System;
using UnityEngine;
using System.Collections.Generic;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;
    
    public class RegistryManager : InternalManager
    {
        #region Static Properties
        public static RegistryManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[RegistryManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
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
        
        private Dictionary<IRegistryIdentifier, IRegistry> registries;
        #endregion

        #region Methods
        public void RegisterRegistry(IRegistry registry)
        {
            IRegistryIdentifier registryIdentifier = registry.RegistryIdentifier;
            if (registries.ContainsKey(registryIdentifier))
            {
                throw new Exception($"[RegistryManager] Registry '{registryIdentifier}' already exists!");
            }

            registries.Add(registryIdentifier, registry);
        }

        public IRegistry GetRegistry(IRegistryIdentifier registryIdentifier)
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

        public IRegistry GetRegistry(RegistryIdentifier registryIdentifier)
        {
            return GetRegistry((IRegistryIdentifier)registryIdentifier);
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            registries = new Dictionary<IRegistryIdentifier, IRegistry>();
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
            looCastUnityInstance = new Instance(this, looCastType);

            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastUnityInstance);
            #endregion
        }
        #endregion
    }
}
