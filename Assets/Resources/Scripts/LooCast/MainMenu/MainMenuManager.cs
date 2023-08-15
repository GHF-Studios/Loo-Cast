using System;
using System.Collections.Generic;

namespace LooCast.MainMenu
{
    using LooCast.System;
    using LooCast.System.ECS;
    using LooCast.Core;

    public sealed class MainMenuManager : ModuleManager
    {
        #region Static Properties
        public static MainMenuManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<MainMenuManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static MainMenuManager instance;
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public MainMenuManager() : base()
        {
            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedMainMenuManagerEntityTypeName = typeof(MainMenuManager).AssemblyQualifiedName;
                string assemblyQualifiedMainMenuManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedMainMenuManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainMenuManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainMenuManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainMenuManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedMainMenuManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedMainMenuManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedMainMenuManagerEntityDataTypeName;
                instanceData.ManagerName = "MainMenuManager";
                instanceData.ManagerParent = LooCastCoreManager.Instance;

                SetEntityMetaData(instanceMetaData);
                SetEntityData(instanceData);

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPreSetup();
                }

                EntityManager.Instance.RegisterEntity(this);
            });

            RegisterSetupAction(() =>
            {
                // Set pre-included components' metaData here

                // Set pre-included component's data here

                // Register pre-included components here

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnPostSetup();
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
        }
        #endregion
    }
}
