using System;
using UnityEngine;

namespace LooCast.System.Management
{
    using LooCast.System.Identification;
    using LooCast.System.Registration;
    using LooCast.System.Resources;
    using LooCast.System.Types;

    public sealed class ResourceManager : InternalManager
    {
        #region Static Properties
        public static ResourceManager Instance
        {
            get
            {
                if (instance == null)
                {
                    UnityEngine.GameObject instanceObject = new UnityEngine.GameObject("[ResourceManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<ResourceManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static ResourceManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private IResourceRegistry<IResourceIdentifier, IResourceIdentifiable> resourceRegistry;
        private IResourceObjectRegistry<IResourceObjectIdentifier, IResourceObjectIdentifiable> resourceObjectRegistry;
        private IResourceFileRegistry<IResourceFileIdentifier, IResourceFileIdentifiable> resourceFileRegistry;
        private IResourceFolderRegistry<IResourceFolderIdentifier, IResourceFolderIdentifiable> resourceFolderRegistry;
        #endregion

        #region Methods
        public void RegisterResource(IResource resource)
        {
            resourceRegistry.Register(resource.ResourceIdentifier, resource);
        }

        public IResource GetResource(IResourceIdentifier resourceIdentifier)
        {
            return (IResource)resourceRegistry.Get(resourceIdentifier);
        }

        public void RegisterResourceObject(IResourceObject resourceObject)
        {
            resourceObjectRegistry.Register(resourceObject.ResourceObjectIdentifier, resourceObject);
            RegisterResource(resourceObject);
        }

        public IResourceObject GetResourceObject(IResourceObjectIdentifier resourceObjectIdentifier)
        {
            return (IResourceObject)resourceObjectRegistry.Get(resourceObjectIdentifier);
        }

        public void RegisterResourceFile(IResourceFile resourceFile)
        {
            resourceFileRegistry.Register(resourceFile.ResourceFileIdentifier, resourceFile);
            RegisterResource(resourceFile);
        }

        public IResourceFile GetResourceFile(IResourceFileIdentifier resourceFileIdentifier)
        {
            return (IResourceFile)resourceFileRegistry.Get(resourceFileIdentifier);
        }

        public void RegisterResourceFolder(IResourceFolder resourceFolder)
        {
            resourceFolderRegistry.Register(resourceFolder.ResourceFolderIdentifier, resourceFolder);
            RegisterResource(resourceFolder);
        }

        public IResourceFolder GetResourceFolder(IResourceFolderIdentifier resourceFolderIdentifier)
        {
            return (IResourceFolder)resourceFolderRegistry.Get(resourceFolderIdentifier);
        }
        #endregion

        #region Overrides
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            #region Namespace/Type/Instance Registration
            TypeManager typeManager = TypeManager.Instance;
            UnityInstanceManager unityInstanceManager = UnityInstanceManager.Instance;

            IUnityInstanceType resourceManagerType = (IUnityInstanceType)typeManager.GetType("LooCast.System.Management:ResourceManager");
            
            UnityInstance resourceManagerInstance = new UnityInstance(this, resourceManagerType);

            unityInstanceManager.RegisterUnityInstance(resourceManagerInstance);
            #endregion
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();

            #region Registry Registration
            TypeManager typeManager = TypeManager.Instance;
            RegistryManager registryManager = RegistryManager.Instance;

            IType iResourceIdentifierType = typeManager.GetType("LooCast.System.Identification:IResourceIdentifier");
            IType iResourceIdentifiableType = typeManager.GetType("LooCast.System.Identification:IResourceIdentifiable");
            IType iResourceObjectIdentifierType = typeManager.GetType("LooCast.System.Identification:IResourceObjectIdentifier");
            IType iResourceObjectIdentifiableType = typeManager.GetType("LooCast.System.Identification:IResourceObjectIdentifiable");
            IType iResourceFileIdentifierType = typeManager.GetType("LooCast.System.Identification:IResourceFileIdentifier");
            IType iResourceFileIdentifiableType = typeManager.GetType("LooCast.System.Identification:IResourceFileIdentifiable");
            IType iResourceFolderIdentifierType = typeManager.GetType("LooCast.System.Identification:IResourceFolderIdentifier");
            IType iResourceFolderIdentifiableType = typeManager.GetType("LooCast.System.Identification:IResourceFolderIdentifiable");

            resourceRegistry = new ResourceRegistry(iResourceIdentifierType, iResourceIdentifiableType);
            resourceObjectRegistry = new ResourceObjectRegistry(iResourceObjectIdentifierType, iResourceObjectIdentifiableType);
            resourceFileRegistry = new ResourceFileRegistry(iResourceFileIdentifierType, iResourceFileIdentifiableType);
            resourceFolderRegistry = new ResourceFolderRegistry(iResourceFolderIdentifierType, iResourceFolderIdentifiableType);

            registryManager.RegisterRegistry(resourceRegistry);
            registryManager.RegisterRegistry(resourceObjectRegistry);
            registryManager.RegisterRegistry(resourceFileRegistry);
            registryManager.RegisterRegistry(resourceFolderRegistry);
            #endregion
        }
        #endregion
    }
}