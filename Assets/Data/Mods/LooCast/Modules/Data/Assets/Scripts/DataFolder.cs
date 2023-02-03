using System;
using System.IO;
using System.Collections.Generic;

namespace LooCast.Data
{
    public class DataFolder : IPersistentDataFolder
    {
        #region Properties
        public string ID
        {
            get
            {
                if (ParentFolder == null)
                {
                    return Name;
                }
                else
                {
                    return ParentFolder.ID + "." + Name;
                }
            }
        }
        public Type DataType { get; }
        public string Name => name;
        public DataFolder ParentFolder => parentFolder;
        public Dictionary<string, IData> ContainedData => containedData;
        public List<DataFolder> ChildFolders => childFolders;
        #endregion

        #region Fields
        private string name;
        private Dictionary<string, IData> containedData;
        private DataFolder parentFolder;
        private List<DataFolder> childFolders;
        #endregion

        #region Constructors
        public DataFolder()
        {
            containedData = new Dictionary<string, IData>();
        }
        #endregion

        #region Methods
        public void AddData(IData data)
        {
            if (data == null)
            {
                throw new ArgumentNullException("data");
            }
            if (containedData.ContainsKey(data.ID))
            {
                throw new ArgumentException($"Data '{data.ID}' already exists!");
            }

            containedData.Add(data.ID, data);
        }

        public void RemoveData(string id)
        {
            if (string.IsNullOrEmpty(id))
            {
                throw new ArgumentNullException("id");
            }
            if (!containedData.ContainsKey(id))
            {
                throw new ArgumentException($"Data '{id}' does not exist!");
            }

            containedData.Remove(id);
        }
        #endregion
    }
}
