using System;
using System.Collections.Generic;

namespace LooCast.Scene
{
    using LooCast.System;
    using LooCast.System.ECS;
    using LooCast.Core;

    public sealed class SceneManager : ModuleManager
    {
        #region Static Properties
        public static SceneManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<SceneManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static SceneManager instance;
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public SceneManager() : base()
        {
            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedSceneManagerEntityTypeName = typeof(SceneManager).AssemblyQualifiedName;
                string assemblyQualifiedSceneManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedSceneManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSceneManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSceneManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSceneManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedSceneManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedSceneManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedSceneManagerEntityDataTypeName;
                instanceData.ManagerName = "SceneManager";
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
