using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;
    using LooCast.Util;

    public class CSharpInstanceManager : InternalManager
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
        private Registry<ICSharpInstanceIdentifier, ICSharpInstanceIdentifiable> csharpInstanceRegistry;
        #endregion

        #region Methods
        public void RegisterInstance(CSharpInstance instance)
        {
            csharpInstanceRegistry.Register(instance.CSharpInstanceIdentifier, instance);
        }

        public void UnregisterInstance(CSharpInstance instance)
        {
            csharpInstanceRegistry.Unregister(instance.CSharpInstanceIdentifier);
        }

        public CSharpInstance GetInstance(CSharpInstanceIdentifier csharpInstanceIdentifier)
        {
            return (CSharpInstance)csharpInstanceRegistry.Get(csharpInstanceIdentifier);
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
            looCastType = new CSharpInstanceType(typeof(CSharpInstanceManager), looCastNamespace);
            looCastUnityInstance = new CSharpInstance(this, (CSharpInstanceType)looCastType);

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
            Type keyType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:ICSharpInstanceIdentifier"));
            Type valueType = typeManager.GetType(new TypeIdentifier("LooCast.System.Identification:ICSharpInstanceIdentifiable"));
            csharpInstanceRegistry = new Registry<ICSharpInstanceIdentifier, ICSharpInstanceIdentifiable>(keyType, valueType);
            registryManager.RegisterRegistry(csharpInstanceRegistry);
            #endregion
        }
        #endregion
    }
}