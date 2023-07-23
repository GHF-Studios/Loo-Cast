using System;

namespace LooCast.Core
{
    using LooCast.System;
    using LooCast.Universe;
    using LooCast.System.ECS;

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
            FolderComponent folderComponent = AddComponent<FolderComponent, Component.MetaData, FolderComponent.Data>();

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

                moduleManagerChildrenList.Add(UniverseManager.Instance);

                foreach (IModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                string assemblyQualifiedFolderComponentTypeName = typeof(FolderComponent).AssemblyQualifiedName;
                string assemblyQualifiedFolderComponentMetaDataTypeName = typeof(Component.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedFolderComponentDataTypeName = typeof(FolderComponent.Data).AssemblyQualifiedName;

                Component.MetaData folderComponentMetaData = new Component.MetaData();
                folderComponentMetaData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                folderComponentMetaData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                folderComponentMetaData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                folderComponentMetaData.ComponentID = new Guid();

                FolderComponent.Data folderComponentData = new FolderComponent.Data();
                folderComponentData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                folderComponentData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                folderComponentData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                folderComponentData.FolderName = "LooCastCoreManager";
                folderComponentData.ParentFolderPath = MainManager.Instance.GetComponent<FolderComponent>().FolderPath;

                folderComponent.SetComponentMetaData(folderComponentMetaData);
                folderComponent.SetComponentData(folderComponentData);

                FolderManager.Instance.RegisterFolder(folderComponent);

                foreach (IModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (IModuleManager moduleManager in moduleManagerChildrenList)
                {
                    moduleManager.OnPostSetup();
                }
            });

            RegisterPreInitializationAction(() =>
            {
                folderComponent.OnPreInitialize();
            });

            RegisterInitializationAction(() =>
            {
                folderComponent.OnInitialize();
            });

            RegisterPostInitializationAction(() =>
            {
                folderComponent.OnPostInitialize();
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
