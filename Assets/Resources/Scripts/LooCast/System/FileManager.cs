using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.Paths;
    using LooCast.System.ECS;

    public sealed class FileManager : ModuleManager
    {
        #region Static Properties
        public static FileManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = Entity.Create<FileManager, Entity.MetaData, Manager.Data>();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static FileManager instance;
        #endregion

        #region Properties
        #endregion

        #region Fields
        private Dictionary<FilePath, IFileComponent> registeredFiles;
        #endregion

        #region Constructors
        public FileManager() : base()
        {
            registeredFiles = new Dictionary<FilePath, IFileComponent>();

            FolderComponent folderComponent = AddComponent<FolderComponent, Component.MetaData, FolderComponent.Data>();

            RegisterPreSetupAction(() =>
            {
                string assemblyQualifiedFileManagerEntityTypeName = typeof(FileManager).AssemblyQualifiedName;
                string assemblyQualifiedFileManagerEntityMetaDataTypeName = typeof(Entity.MetaData).AssemblyQualifiedName;
                string assemblyQualifiedFileManagerEntityDataTypeName = typeof(Manager.Data).AssemblyQualifiedName;

                Entity.MetaData fileManagerMetaData = new Entity.MetaData();
                fileManagerMetaData.AssemblyQualifiedEntityTypeName = assemblyQualifiedFileManagerEntityTypeName;
                fileManagerMetaData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedFileManagerEntityMetaDataTypeName;
                fileManagerMetaData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedFileManagerEntityDataTypeName;
                fileManagerMetaData.EntityID = new Guid();

                Manager.Data fileManagerData = new Manager.Data();
                fileManagerData.AssemblyQualifiedEntityTypeName = assemblyQualifiedFileManagerEntityTypeName;
                fileManagerData.AssemblyQualifiedEntityMetaDataTypeName = assemblyQualifiedFileManagerEntityMetaDataTypeName;
                fileManagerData.AssemblyQualifiedEntityDataTypeName = assemblyQualifiedFileManagerEntityDataTypeName;
                fileManagerData.ManagerName = "FileManager";
                fileManagerData.ManagerParent = SystemManager.Instance;

                SetEntityMetaData(fileManagerMetaData);
                SetEntityData(fileManagerData);

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
                folderComponentData.FolderName = "FileManager";
                folderComponentData.ParentFolderPath = SystemManager.Instance.GetComponent<FolderComponent>().FolderPath;

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

        #region Methods
        public void RegisterFile(IFileComponent file)
        {
            if (!registeredFiles.ContainsKey(file.FilePath))
            {
                registeredFiles.Add(file.FilePath, file);
            }
        }

        public void UnregisterFile(IFileComponent file)
        {
            if (registeredFiles.ContainsKey(file.FilePath))
            {
                registeredFiles.Remove(file.FilePath);
            }
        }

        public IFileComponent GetFile(FilePath filePath)
        {
            if (registeredFiles.ContainsKey(filePath))
            {
                return registeredFiles[filePath];
            }
            return null;
        }

        public bool TryGetFile(FilePath filePath, out IFileComponent file)
        {
            if (!registeredFiles.ContainsKey(filePath))
            {
                file = null;
                return false;
            }
            else
            {
                file = registeredFiles[filePath];
                return true;
            }
        }

        public IFileComponent GetFile(string fileGUSP)
        {
            if (!FilePath.TryParse(fileGUSP, out FilePath? filePath))
            {
                return null;
            }
            return GetFile((FilePath)filePath);
        }

        public bool TryGetFile(string stringFilePath, out IFileComponent file)
        {
            if (!FilePath.TryParse(stringFilePath, out FilePath? filePath))
            {
                file = null;
                return false;
            }
            return TryGetFile((FilePath)filePath, out file);
        }

        public bool IsFileRegistered(FilePath filePath)
        {
            return registeredFiles.ContainsKey(filePath);
        }
        #endregion
    }
}
