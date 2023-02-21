using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;

    public sealed class UnityInstanceManager : InternalManager
    {
        #region Static Properties
        public static UnityInstanceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[UnityInstanceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<UnityInstanceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UnityInstanceManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private UnityInstanceRegistry unityInstanceRegistry;
        #endregion

        #region Methods
        public void RegisterUnityInstance(IUnityInstance instance)
        {
            unityInstanceRegistry.Register(instance.UnityInstanceIdentifier, instance);
        }

        public void UnregisterUnityInstance(IUnityInstance instance)
        {
            unityInstanceRegistry.Unregister(instance.UnityInstanceIdentifier);
        }

        public IUnityInstance GetUnityInstance(IUnityInstanceIdentifier unityInstanceIdentifier)
        {
            return (IUnityInstance)unityInstanceRegistry.Get(unityInstanceIdentifier);
        }

        public IUnityInstance GetUnityInstance(UnityInstanceIdentifier unityInstanceIdentifier)
        {
            return GetUnityInstance(unityInstanceIdentifier);
        }
        #endregion

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            looCastNamespace = namespaceManager.GetNamespace("LooCast");
            looCastType = new UnityInstanceType(typeof(UnityInstanceManager), looCastNamespace);
            looCastUnityInstance = new UnityInstance(this, (UnityInstanceType)looCastType);

            typeManager.RegisterType(looCastType);
            unityInstanceManager.RegisterUnityInstance(looCastUnityInstance);
            #endregion
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Registry Registration
            RegistryManager registryManager = RegistryManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            
            IType keyType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:IUnityInstanceIdentifier"));
            IType valueType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:IUnityInstanceIdentifiable"));
            unityInstanceRegistry = new UnityInstanceRegistry(keyType, valueType);
            
            registryManager.RegisterRegistry(unityInstanceRegistry);
            #endregion
        }
        #endregion
    }
}