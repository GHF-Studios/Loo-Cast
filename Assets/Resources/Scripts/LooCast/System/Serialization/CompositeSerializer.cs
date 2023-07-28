using System;
using System.Collections.Generic;
using System.IO;

namespace LooCast.System.Serialization
{
    public abstract class CompositeSerializer<SerializableType, SerializedType> : Serializer
    {
        #region Properties
        public StorageType StorageType { get; private set; }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        public CompositeSerializer() : base(SerializationType.Composite, typeof(SerializableType), typeof(SerializedType))
        {
            if (typeof(FileInfo).IsAssignableFrom(this.SerializedType))
            {
                StorageType = StorageType.File;
            }
            else if (typeof(DirectoryInfo).IsAssignableFrom(this.SerializedType))
            {
                StorageType = StorageType.Folder;
            }
            else if (typeof(ObjectInfo).IsAssignableFrom(this.SerializedType))
            {

            }
        }
        #endregion

        #region Methods
        #endregion
    }
}
