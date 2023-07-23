using System;
using System.Collections.Generic;

namespace LooCast.Universe
{
    using LooCast.System;
    using LooCast.Core;
    using LooCast.System.ECS;

    public sealed class UniverseManager : ModuleManager
    {
        #region Static Properties
        public static UniverseManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<UniverseManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static UniverseManager instance;
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public UniverseManager() : base()
        {
            FolderComponent folderComponent = AddComponent<FolderComponent, Component.MetaData, FolderComponent.Data>();

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedUniverseManagerEntityTypeName = typeof(UniverseManager).AssemblyQualifiedName;
                string assemblyQualifiedUniverseManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedUniverseManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedUniverseManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedUniverseManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedUniverseManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedUniverseManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedUniverseManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedUniverseManagerEntityDataTypeName;
                instanceData.ManagerName = "UniverseManager";
                instanceData.ManagerParent = LooCastCoreManager.Instance;

                SetEntityMetaData(instanceMetaData);
                SetEntityData(instanceData);

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
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
                folderComponentData.FolderName = "UniverseManager";
                folderComponentData.ParentFolderPath = LooCastCoreManager.Instance.GetComponent<FolderComponent>().FolderPath;

                folderComponent.SetComponentMetaData(folderComponentMetaData);
                folderComponent.SetComponentData(folderComponentData);

                FolderManager.Instance.RegisterFolder(folderComponent);

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPostSetup();
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
