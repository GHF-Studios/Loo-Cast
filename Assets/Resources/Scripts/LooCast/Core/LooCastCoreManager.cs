using System;

namespace LooCast.Core
{
    using LooCast.System;
    using LooCast.System.ECS;
    using LooCast.Steam;
    using LooCast.Save;
    using LooCast.Scene;
    using LooCast.MainMenu;
    using LooCast.Game;
    using LooCast.Universe;

    public sealed class LooCastCoreManager : CoreModuleManager
    {
        #region Static Properties
        public static LooCastCoreManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<LooCastCoreManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static LooCastCoreManager instance;
        #endregion

        #region Properties
        public Universe Universe { get; private set; }
        public UniverseObserver UniverseObserver { get; private set; }
        #endregion

        #region Constructors
        public LooCastCoreManager() : base()
        {
            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedLooCastCoreManagerEntityTypeName = typeof(LooCastCoreManager).AssemblyQualifiedName;
                string assemblyQualifiedLooCastCoreManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedLooCastCoreManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData looCastCoreManagerMetaData = new Entity.MetaData();
                looCastCoreManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedLooCastCoreManagerEntityTypeName;
                looCastCoreManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedLooCastCoreManagerEntityMetaDataTypeName;
                looCastCoreManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedLooCastCoreManagerEntityDataTypeName;
                looCastCoreManagerMetaData.EntityID = new Guid();

                Manager.Data looCastCoreManagerData = new Manager.Data();
                looCastCoreManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedLooCastCoreManagerEntityTypeName;
                looCastCoreManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedLooCastCoreManagerEntityMetaDataTypeName;
                looCastCoreManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedLooCastCoreManagerEntityDataTypeName;
                looCastCoreManagerData.ManagerName = "LooCastCoreManager";
                looCastCoreManagerData.ManagerParent = MainManager.Instance;

                SetEntityMetaData(looCastCoreManagerMetaData);
                SetEntityData(looCastCoreManagerData);

                moduleManagerChildrenList.Add(SteamManager.Instance);
                moduleManagerChildrenList.Add(SaveManager.Instance);
                moduleManagerChildrenList.Add(SceneManager.Instance);
                moduleManagerChildrenList.Add(MainMenuManager.Instance);
                moduleManagerChildrenList.Add(GameManager.Instance);
                moduleManagerChildrenList.Add(UniverseManager.Instance);

                foreach (ModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component's data here

                // Register pre-included components here

                foreach (ModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (ModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnPostSetup();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                // Pre-Initialize pre-included components here
            });

            RegisterInitializationAction(() =>
            {
                // Initialize pre-included components here
            });

            RegisterPostInitializationAction(() =>
            {
                // Post-Initialize pre-included components here
            });

            RegisterLatePostInitializationAction(() =>
            {
                Universe = new Universe(32);
                UniverseObserver = new UniverseObserver(256);
            });
        }
        #endregion
    }
}
