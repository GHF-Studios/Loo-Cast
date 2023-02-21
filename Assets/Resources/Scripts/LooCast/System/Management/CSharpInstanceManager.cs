using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;

    public sealed class CSharpInstanceManager : InternalManager
    {
        #region Static Properties
        public static CSharpInstanceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[CSharpInstanceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<CSharpInstanceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static CSharpInstanceManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private CSharpInstanceRegistry csharpInstanceRegistry;
        #endregion

        #region Methods
        public void RegisterCSharpInstance(ICSharpInstance instance)
        {
            csharpInstanceRegistry.Register(instance.CSharpInstanceIdentifier, instance);
        }

        public void UnregisterCSharpInstance(ICSharpInstance instance)
        {
            csharpInstanceRegistry.Unregister(instance.CSharpInstanceIdentifier);
        }

        public ICSharpInstance GetCSharpInstance(ICSharpInstanceIdentifier csharpInstanceIdentifier)
        {
            return (ICSharpInstance)csharpInstanceRegistry.Get(csharpInstanceIdentifier);
        }

        public ICSharpInstance GetCSharpInstance(CSharpInstanceIdentifier csharpInstanceIdentifier)
        {
            return GetCSharpInstance(csharpInstanceIdentifier);
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
            looCastType = new UnityInstanceType(typeof(CSharpInstanceManager), looCastNamespace);
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
            
            IType keyType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:ICSharpInstanceIdentifier"));
            IType valueType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:ICSharpInstanceIdentifiable"));
            csharpInstanceRegistry = new CSharpInstanceRegistry(keyType, valueType);
            
            registryManager.RegisterRegistry(csharpInstanceRegistry);
            #endregion
        }
        #endregion
    }
}