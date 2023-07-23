using System;

namespace LooCast.System
{
    using LooCast.System.ECS;
    
    public sealed class SystemManager : CoreModuleManager
    {
        #region Static Properties
        public static SystemManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<SystemManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static SystemManager instance;
        #endregion

        #region Constructors
        public SystemManager() : base()
        {
            FolderComponent folderComponent = AddComponent<FolderComponent, Component.MetaData, FolderComponent.Data>();

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedSystemManagerEntityTypeName = typeof(SystemManager).AssemblyQualifiedName;
                string assemblyQualifiedSystemManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedSystemManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData systemManagerMetaData = new Entity.MetaData();
                systemManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSystemManagerEntityTypeName;
                systemManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSystemManagerEntityMetaDataTypeName;
                systemManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSystemManagerEntityDataTypeName;
                systemManagerMetaData.EntityID = new Guid();

                Manager.Data systemManagerData = new Manager.Data();
                systemManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSystemManagerEntityTypeName;
                systemManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSystemManagerEntityMetaDataTypeName;
                systemManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSystemManagerEntityDataTypeName;
                systemManagerData.ManagerName = "SystemManager";
                systemManagerData.ManagerParent = MainManager.Instance;

                SetEntityMetaData(systemManagerMetaData);
                SetEntityData(systemManagerData);

                moduleManagerChildrenList.Add(FolderManager.Instance);
                moduleManagerChildrenList.Add(FileManager.Instance);
                moduleManagerChildrenList.Add(ObjectManager.Instance);
                moduleManagerChildrenList.Add(EntityManager.Instance);
                moduleManagerChildrenList.Add(ComponentManager.Instance);

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
                folderComponentData.FolderName = "SystemManager";
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
        }
        #endregion
    }
}
