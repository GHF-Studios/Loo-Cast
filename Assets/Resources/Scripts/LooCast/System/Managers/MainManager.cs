using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

namespace LooCast.System.Managers
{
    using global::LooCast.System.Registries;
    
    public sealed class MainManager : InternalManager
    {
        #region Static Properties
        public static MainManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new MainManager();
                    instance.UnityEngineGameObject.name = "[MainManager]";
                    instance.UnityEngineGameObject.layer = 31;
                    instance.UnityEngineGameObject.tag = "INTERNAL";
                    instance.UnityEngineGameObject.transform.parent = LooCast.Instance.gameObject.transform;
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static MainManager instance;
        #endregion

        #region Properties
        public CoreModuleManager[] CoreModuleManagers => coreModuleManagers;
        public MainRegistry MainRegistry => mainRegistry;
        #endregion

        #region Fields
        private CoreModuleManager[] coreModuleManagers;
        private MainRegistry mainRegistry;
        #endregion

        #region Constructors
        public MainManager() : base("LooCast.System:MainManager", null)
        {
            coreModuleManagers = new CoreModuleManager[]
            {
                Core.CoreManager.Instance
            };
            mainRegistry = new MainRegistry();

            RegisterEarlyPreInitializationAction(() => 
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.EarlyPreInitialize();
                }
            });
            RegisterPreInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.PreInitialize();
                }
            });
            RegisterLatePreInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.LatePreInitialize();
                }
            });
            RegisterEarlyInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.EarlyInitialize();
                }
            });
            RegisterInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.Initialize();
                }
            });
            RegisterLateInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.LateInitialize();
                }
            });
            RegisterEarlyPostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.EarlyPostInitalize();
                }
            });
            RegisterPostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.PostInitialize();
                }
            });
            RegisterLatePostInitializationAction(() =>
            {
                foreach (CoreModuleManager coreModuleManager in coreModuleManagers)
                {
                    coreModuleManager.LatePostInitialize();
                }
            });
        }
        #endregion

        #region Methods
        public void RegisterIdentifiable(ILooCastObject identifiable)
        {
            if (identifiable == null)
            {
                throw new ArgumentNullException(nameof(identifiable));
            }

            if (identifiable.Identifier == null)
            {
                throw new ArgumentException("The identifiable does not have an identifier.", nameof(identifiable));
            }

            mainRegistry.Add(identifiable.Identifier, identifiable);
        }
        
        public void UnregisterIdentifiable(Identifier identifier)
        {
            if (identifier == null)
            {
                throw new ArgumentNullException(nameof(identifier));
            }

            mainRegistry.Remove(identifier);
        }

        public ILooCastObject GetIdentifiable(Identifier identifier)
        {
            if (identifier == null)
            {
                throw new ArgumentNullException(nameof(identifier));
            }

            if (!mainRegistry.TryGetValue(identifier, out ILooCastObject identifiable))
            {
                throw new ArgumentException($"The identifiable with identifier {identifier} could not be found in the main registry.");
            }

            return identifiable;
        }
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            RegistryManager registryManager = RegistryManager.Instance;
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            SystemObjectManager systemObjectManager = SystemObjectManager.Instance;
            GameObjectManager gameObjectManager = GameObjectManager.Instance;
            ComponentManager componentManager = ComponentManager.Instance;

            #region Namespaces
            Namespace looCastNamespace = new Namespace("LooCast");
            namespaceManager.RegisterNamespace(looCastNamespace);

            Namespace looCastSystemNamespace = new Namespace("System", looCastNamespace);
            namespaceManager.RegisterNamespace(looCastSystemNamespace);

            Namespace looCastSystemCollectionsNamespace = new Namespace("Collections", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemCollectionsNamespace);

            Namespace looCastSystemCollectionsConcurrentNamespace = new Namespace("Concurrent", looCastSystemCollectionsNamespace);
            Namespace looCastSystemCollectionsGenericNamespace = new Namespace("Generic", looCastSystemCollectionsNamespace);
            namespaceManager.RegisterNamespace(looCastSystemCollectionsConcurrentNamespace);
            namespaceManager.RegisterNamespace(looCastSystemCollectionsGenericNamespace);

            Namespace looCastSystemExceptionsNamespace = new Namespace("Exceptions", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemExceptionsNamespace);

            Namespace looCastSystemHierarchiesNamespace = new Namespace("Hierarchies", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemHierarchiesNamespace);

            Namespace looCastSystemIdentifiersNamespace = new Namespace("Identifiers", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemIdentifiersNamespace);

            Namespace looCastSystemManagersNamespace = new Namespace("Managers", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemManagersNamespace);

            Namespace looCastSystemRegistriesNamespace = new Namespace("Registries", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemRegistriesNamespace);

            Namespace looCastSystemResourcesNamespace = new Namespace("Resources", looCastSystemNamespace);
            namespaceManager.RegisterNamespace(looCastSystemResourcesNamespace);
            #endregion

            #region Types
            // LooCast
            Type looCastType = new Type(typeof(LooCast));
            typeManager.RegisterType(looCastType);

            // LooCast.System
            Type namespaceType = new Type(typeof(Namespace));
            Type typeType = new Type(typeof(Type));             // I love my naming convention, but it's like Einstein's Theories: It is not always right; it has these singularities where it fails and this is definitely one of these singularities LMAO
            Type systemObjectType = new Type(typeof(SystemObject));
            Type gameObjectType = new Type(typeof(GameObject));
            Type componentType = new Type(typeof(Component));

            Type managerType = new Type(typeof(Manager));
            Type internalManagerType = new Type(typeof(InternalManager));
            Type moduleManagerType = new Type(typeof(ModuleManager));
            Type subModuleManagerType = new Type(typeof(SubModuleManager));
            Type coreModuleManagerType = new Type(typeof(CoreModuleManager));

            Type identifierType = new Type(typeof(Identifier));
            Type iIdentifiableType = new Type(typeof(ILooCastObject));
            Type registryType = new Type(typeof(Registry<Identifier, ILooCastObject>));
            
            Type iHierarchyElementType = new Type(typeof(HierarchyElement));
            Type hierarchyElementPathType = new Type(typeof(HierarchyElementPath));
            Type hierarchyType = new Type(typeof(Hierarchy<HierarchyElement>));
            #endregion
        }

        public override void InitializeInstance()
        {
            base.InitializeInstance();
        }

        public override void PostInitializeInstance()
        {
            base.PostInitializeInstance();
        }
        #endregion
    }
}
