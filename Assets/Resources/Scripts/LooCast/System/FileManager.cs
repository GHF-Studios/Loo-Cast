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
                    string assemblyQualifiedEntityTypeName = typeof(FileManager).AssemblyQualifiedName;
                    instance = Entity.Create<FileManager>();

                    Entity.MetaData instanceMetaData = new Entity.MetaData
                        (
                            assemblyQualifiedEntityTypeName,
                            new Guid(),
                            new IComponent.IMetaData[]
                            {
                                new FolderComponent.MetaData(typeof(FolderComponent).AssemblyQualifiedName)
                            }
                        );

                    Manager.Data instanceData = new Manager.Data
                        (
                            assemblyQualifiedEntityTypeName,
                            new IComponent.IData[]
                            {
                                new FolderComponent.Data
                                    (
                                        typeof(FolderComponent).AssemblyQualifiedName,
                                        "FileManager",
                                        SystemManager.Instance.GetComponent<FolderComponent>().FolderPath
                                    )
                            },
                            "FileManager",
                            SystemManager.Instance
                        );


                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetMetaData(instanceMetaData);
                    ((ISerializable<Entity.MetaData, Manager.Data>)instance).SetData(instanceData);
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
        private Dictionary<FilePath, IFile> registeredFiles;
        #endregion

        #region Constructors
        public FileManager() : base()
        {
            registeredFiles = new Dictionary<FilePath, IFile>();
        }
        #endregion

        #region Methods
        public void RegisterFile(IFile file)
        {
            if (!registeredFiles.ContainsKey(file.FilePath))
            {
                registeredFiles.Add(file.FilePath, file);
            }
        }

        public void UnregisterFile(IFile file)
        {
            if (registeredFiles.ContainsKey(file.FilePath))
            {
                registeredFiles.Remove(file.FilePath);
            }
        }

        public IFile GetFile(FilePath filePath)
        {
            if (registeredFiles.ContainsKey(filePath))
            {
                return registeredFiles[filePath];
            }
            return null;
        }

        public bool TryGetFile(FilePath filePath, out IFile file)
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

        public IFile GetFile(string fileGUSP)
        {
            if (!FilePath.TryParse(fileGUSP, out FilePath? filePath))
            {
                return null;
            }
            return GetFile(filePath!);
        }

        public bool TryGetFile(string stringFilePath, out IFile file)
        {
            if (!FilePath.TryParse(stringFilePath, out FilePath? filePath))
            {
                file = null;
                return false;
            }
            return TryGetFile(filePath!, out file);
        }

        public bool IsFileRegistered(FilePath filePath)
        {
            return registeredFiles.ContainsKey(filePath);
        }
        #endregion
    }
}
