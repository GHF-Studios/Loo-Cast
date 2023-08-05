using System;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    public class SerializableFileTypeInfo : SerializableTypeInfo
    {
        #region Properties
        public Type SerializableFileType { get; private set; }
        public HashSet<SerializableObjectTypeInfo> SubSerializableObjectTypeInfos { get; private set; }
        #endregion

        #region Constructors
        public SerializableFileTypeInfo(Type serializableFileType, HashSet<SerializableObjectTypeInfo> subSerializableObjectTypeInfos)
        {
            SerializableFileType = serializableFileType;
            SubSerializableObjectTypeInfos = subSerializableObjectTypeInfos;
        }
        #endregion
    }
}
