using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.Paths;
    using LooCast.System.Serialization;
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

                    string assemblyQualifiedMainManagerEntityTypeName = typeof(FileManager).AssemblyQualifiedName;
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
                    folderComponentData.FolderName = "FileManager";
                    folderComponentData.ParentFolderPath = SystemManager.Instance.GetComponent<FolderComponent>().FolderPath;
                    instanceData.ComponentDatas = new IComponent.IData[]
                    {
                        folderComponentData
                    };
                    instanceData.ManagerName = "FileManager";
                    instanceData.ManagerParent = SystemManager.Instance;

                    instance.SetMetaData(instanceMetaData);
                    instance.SetData(instanceData);
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
