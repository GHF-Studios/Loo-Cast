using System;
using System.Collections.Generic;

namespace LooCast.Game
{
    using LooCast.System;
    using LooCast.System.ECS;
    using LooCast.Core;

    public sealed class GameManager : ModuleManager
    {
        #region Static Properties
        public static GameManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<GameManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static GameManager instance;
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public GameManager() : base()
        {
            // Add pre-included components here

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedGameManagerEntityTypeName = typeof(GameManager).AssemblyQualifiedName;
                string assemblyQualifiedGameManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedGameManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData instanceMetaData = new Entity.MetaData();
                instanceMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedGameManagerEntityTypeName;
                instanceMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedGameManagerEntityMetaDataTypeName;
                instanceMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedGameManagerEntityDataTypeName;
                instanceMetaData.EntityID = new Guid();

                Manager.Data instanceData = new Manager.Data();
                instanceData.AssemblyQualifiedEntityTypeName = assemblyQualifiedGameManagerEntityTypeName;
                instanceData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedGameManagerEntityMetaDataTypeName;
                instanceData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedGameManagerEntityDataTypeName;
                instanceData.ManagerName = "GameManager";
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
<<<<<<< HEAD:Assets/Resources/Scripts/LooCast/Game/GameManager.cs
                // Set pre-included components' metaData here

                // Set pre-included component's data here

                // Register pre-included components here

                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
=======
                return;
            }
            if (!Instance.IsPaused)
            {
                Instance.IsPaused = true;
                foreach (Component extendedMonoBehaviour in Component.Instances)
>>>>>>> develop:Assets/Mods/LooCast/Modules/Game/Assets/Scripts/GameManager.cs
                {
                    subModuleManager.OnSetup();
                }
            });

            RegisterPostSetupAction(() =>
            {
<<<<<<< HEAD:Assets/Resources/Scripts/LooCast/Game/GameManager.cs
                foreach (SubModuleManager subModuleManager in subModuleManagerChildrenList)
=======
                return;
            }
            if (Instance.IsPaused)
            {
                Instance.IsPaused = false;
                foreach (Component extendedMonoBehaviour in Component.Instances)
>>>>>>> develop:Assets/Mods/LooCast/Modules/Game/Assets/Scripts/GameManager.cs
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
