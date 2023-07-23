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
                    instance = Entity.Create<FolderManager, Entity.MetaData, Manager.Data>();

                    string assemblyQualifiedMainManagerEntityTypeName = typeof(FolderManager).AssemblyQualifiedName;
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
                    folderComponentData.FolderName = "FolderManager";
                    folderComponentData.ParentFolderPath = SystemManager.Instance.GetComponent<FolderComponent>().FolderPath;
                    instanceData.ComponentDatas = new IComponent.IData[]
                    {
                        folderComponentData
                    };
                    instanceData.ManagerName = "FolderManager";
                    instanceData.ManagerParent = SystemManager.Instance;

                    instance.SetMetaData(instanceMetaData);
                    instance.SetData(instanceData);
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
