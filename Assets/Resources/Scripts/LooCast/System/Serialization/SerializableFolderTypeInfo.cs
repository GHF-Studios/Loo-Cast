using System;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    public class SerializableFolderTypeInfo : SerializableTypeInfo
    {
        #region Properties
        public Type SerializableFolderType { get; private set; }
        public HashSet<SerializableFileTypeInfo> SubSerializableFileTypeInfos { get; private set; }
        public HashSet<SerializableFolderTypeInfo> SubSerializableFolderTypeInfos { get; private set; }
        #endregion

        #region Constructors
        public SerializableFolderTypeInfo(Type serializableFolderType, HashSet<SerializableFileTypeInfo> subSerializableFileTypeInfos, HashSet<SerializableFolderTypeInfo> subSerializableFolderTypeInfos)
        {
            SerializableFolderType = serializableFolderType;
            SubSerializableFileTypeInfos = subSerializableFileTypeInfos;
            SubSerializableFolderTypeInfos = subSerializableFolderTypeInfos;
        }
        #endregion
    }
}
