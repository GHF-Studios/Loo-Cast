using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.Paths;
    using LooCast.System.Serialization;
    using LooCast.System.ECS;

    public sealed class FolderManager : ModuleManager
    {
        #region Static Properties
        public static FolderManager Instance
        {
            get
            {
                if (instance == null)
                {
                    string assemblyQualifiedEntityTypeName = typeof(FolderManager).AssemblyQualifiedName;
                    instance = Entity.Create<FolderManager>();

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
                                        "FolderManager",
                                        SystemManager.Instance.GetComponent<FolderComponent>().FolderPath
                                    )
                            },
                            "FolderManager",
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
            return GetFolder(folderPath!);
        }

        public bool TryGetFolder(string stringFolderPath, out IFolderComponent folder)
        {
            if (!FolderPath.TryParse(stringFolderPath, out FolderPath? folderPath))
            {
                folder = null;
                return false;
            }
            return TryGetFolder(folderPath!, out folder);
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
