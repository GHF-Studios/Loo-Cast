using System;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    public class SerializableFolderTypeInfo : SerializableTypeInfo
    {
        #region Properties
        public Type SerializableFolderType { get; private set; }
        public HashSet<SerializableFileTypeInfo> SubSerializableFileTypes { get; private set; }
        public HashSet<SerializableFolderTypeInfo> SubSerializableFolderTypes { get; private set; }
        #endregion

        #region Constructors
        public SerializableFolderTypeInfo(Type serializableFolderType, HashSet<SerializableFileTypeInfo> subSerializableFileTypes, HashSet<SerializableFolderTypeInfo> subSerializableFolderTypes)
        {
            SerializableFolderType = serializableFolderType;
            SubSerializableFileTypes = subSerializableFileTypes;
            SubSerializableFolderTypes = subSerializableFolderTypes;
        }
        #endregion
    }
}
