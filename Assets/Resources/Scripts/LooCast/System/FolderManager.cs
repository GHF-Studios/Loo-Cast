using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.Paths;
    using LooCast.System.ECS;
    using LooCast.Core;

    public sealed class FolderManager : ModuleManager
    {
        #region Static Properties
        public static FolderManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<FolderManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static FolderManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private Dictionary<FolderPath, IFolderComponent> registeredFolders;
        #endregion

        #region Constructors
        public FolderManager() : base()
        {
            registeredFolders = new Dictionary<FolderPath, IFolderComponent>();

            FolderComponent folderComponent = AddComponent<FolderComponent, Component.MetaData, FolderComponent.Data>();

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedFolderManagerEntityTypeName = typeof(FolderManager).AssemblyQualifiedName;
                string assemblyQualifiedFolderManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedFolderManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData folderManagerMetaData = new Entity.MetaData();
                folderManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedFolderManagerEntityTypeName;
                folderManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedFolderManagerEntityMetaDataTypeName;
                folderManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedFolderManagerEntityDataTypeName;
                folderManagerMetaData.EntityID = new Guid();

                Manager.Data folderManagerData = new Manager.Data();
                folderManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedFolderManagerEntityTypeName;
                folderManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedFolderManagerEntityMetaDataTypeName;
                folderManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedFolderManagerEntityDataTypeName;
                folderManagerData.ManagerName = "FolderManager";
                folderManagerData.ManagerParent = LooCastCoreManager.Instance;

                SetEntityMetaData(folderManagerMetaData);
                SetEntityData(folderManagerData);

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
                folderComponentData.FolderName = "FolderManager";
                folderComponentData.ParentFolderPath = LooCastCoreManager.Instance.GetComponent<FolderComponent>().FolderPath;

                folderComponent.SetComponentMetaData(folderComponentMetaData);
                folderComponent.SetComponentData(folderComponentData);

                foreach (ISubModuleManager subModuleManager in subModuleManagerChildrenList)
                {
                    subModuleManager.OnSetup();
                }

                FolderManager.Instance.RegisterFolder(folderComponent);
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

        #region Methods
        public void RegisterFolder(IFolderComponent folder)
        {
            if (!registeredFolders.ContainsKey(folder.FolderPath))
            {
                registeredFolders.Add(folder.FolderPath, folder);
            }
        }

        public void UnregisterFolder(IFolderComponent folder)
        {
            if (registeredFolders.ContainsKey(folder.FolderPath))
            {
                registeredFolders.Remove(folder.FolderPath);
            }
        }

        public IFolderComponent GetFolder(FolderPath folderPath)
        {
            if (registeredFolders.ContainsKey(folderPath))
            {
                return registeredFolders[folderPath];
            }
            return null;
        }

        public bool TryGetFolder(FolderPath folderPath, out IFolderComponent folder)
        {
            if (folderPath == "/")
            {
                folder = MainManager.Instance.GetComponent<FolderComponent>();
                return true;
            }
            if (!registeredFolders.ContainsKey(folderPath))
            {
                folder = null;
                return false;
            }
            else
            {
                folder = registeredFolders[folderPath];
                return true;
            }
        }

        public IFolderComponent GetFolder(string folderGUSP)
        {
            if (!FolderPath.TryParse(folderGUSP, out FolderPath? folderPath))
            {
                return null;
            }
            return GetFolder((FolderPath)folderPath);
        }

        public bool TryGetFolder(string stringFolderPath, out IFolderComponent folder)
        {
            if (!FolderPath.TryParse(stringFolderPath, out FolderPath? folderPath))
            {
                folder = null;
                return false;
            }
            return TryGetFolder((FolderPath)folderPath, out folder);
        }

        public bool IsFolderRegistered(FolderPath folderPath)
        {
            if (folderPath == "/")
            {
                return true;
            }
            return registeredFolders.ContainsKey(folderPath);
        }
        #endregion
    }
}
