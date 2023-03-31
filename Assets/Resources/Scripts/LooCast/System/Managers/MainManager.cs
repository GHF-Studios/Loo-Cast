using System;
using System.Linq;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;
using Assets.Resources.Scripts.LooCast.System.Managers;

namespace LooCast.System.Managers
{
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
                    instance.GameObjectInstance.name = "[MainManager]";
                    instance.GameObjectInstance.layer = 31;
                    instance.GameObjectInstance.tag = "INTERNAL";
                    instance.GameObjectInstance.transform.parent = LooCast.Instance.gameObject.transform;
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static MainManager instance;
        #endregion

        #region Fields
        private CoreModuleManager[] coreModuleManagers;
        #endregion

        #region Constructors
        public MainManager() : base("LooCast.System:MainManager", null)
        {
            coreModuleManagers = new CoreModuleManager[]
            {
                Core.CoreManager.Instance
            };
            
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
            Type iIdentifiableType = new Type(typeof(IIdentifiable));
            Type registryType = new Type(typeof(Registry<Identifier, IIdentifiable>));
            
            Type iHierarchyElementType = new Type(typeof(IHierarchyElement));
            Type hierarchyPathType = new Type(typeof(HierarchyPath));
            Type hierarchyType = new Type(typeof(Hierarchy<IHierarchyElement>));
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
