using System;

namespace LooCast.System
{
    using LooCast.System.Serialization;
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

                    string assemblyQualifiedMainManagerEntityTypeName = typeof(SystemManager).AssemblyQualifiedName;
                    string assemblyQualifiedMainManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                    string assemblyQualifiedMainManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                    string assemblyQualifiedFolderComponentTypeName = typeof(FolderComponent).AssemblyQualifiedName;
                    string assemblyQualifiedFolderComponentMetaDataTypeName = typeof(Component.MetaData).AssemblyQualifiedName;
                    string assemblyQualifiedFolderComponentDataTypeName = typeof(FolderComponent.Data).AssemblyQualifiedName;

                    Entity.MetaData instanceMetaData = new Entity.MetaData();
                    instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainManagerEntityTypeName;
                    instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainManagerEntityMetaDataTypeName;
                    instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainManagerEntityDataTypeName;
                    instanceMetaData.GUID = new Guid();
                    IFolderComponent.IMetaData folderComponentMetaData = new FolderComponent.MetaData();
                    folderComponentMetaData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                    folderComponentMetaData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                    folderComponentMetaData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                    folderComponentMetaData.GUID = new Guid();
                    instanceMetaData.ComponentMetaDatas = new IComponent.IMetaData[]
                    {
                        folderComponentMetaData
                    };

                    Manager.Data instanceData = new Manager.Data();
                    instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainManagerEntityTypeName;
                    instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainManagerEntityMetaDataTypeName;
                    instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainManagerEntityDataTypeName;
                    IFolderComponent.IData folderComponentData = new FolderComponent.Data();
                    folderComponentData.AssemblyQualifiedComponentTypeName = assemblyQualifiedFolderComponentTypeName;
                    folderComponentData.AssemblyQualifiedComponentMetaDataTypeName = assemblyQualifiedFolderComponentMetaDataTypeName;
                    folderComponentData.AssemblyQualifiedComponentDataTypeName = assemblyQualifiedFolderComponentDataTypeName;
                    folderComponentData.FolderName = "SystemManager";
                    folderComponentData.ParentFolderPath = MainManager.Instance.GetComponent<FolderComponent>().FolderPath;
                    instanceData.ComponentDatas = new IComponent.IData[]
                    {
                        folderComponentData
                    };
                    instanceData.ManagerName = "SystemManager";
                    instanceData.ManagerParent = MainManager.Instance;

                    instance.SetMetaData(instanceMetaData);
                    instance.SetData(instanceData);
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
            RegisterSetupAction(() =>
            {
                AddChildModuleManager(FolderManager.Instance);
                AddChildModuleManager(FileManager.Instance);
                AddChildModuleManager(ObjectManager.Instance);
                AddChildModuleManager(EntityManager.Instance); ;
                AddChildModuleManager(ComponentManager.Instance);
            });
        }
        #endregion
    }
}
