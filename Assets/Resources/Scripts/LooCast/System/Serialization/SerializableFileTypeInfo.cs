using System;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    public class SerializableFileTypeInfo : SerializableTypeInfo
    {
        #region Properties
        public Type SerializableFileType { get; private set; }
        public HashSet<SerializableObjectTypeInfo> SubSerializableObjectTypes { get; private set; }
        #endregion

        #region Constructors
        public SerializableFileTypeInfo(Type serializableFileType, HashSet<SerializableObjectTypeInfo> subSerializableObjectTypes)
        {
            SerializableFileType = serializableFileType;
            SubSerializableObjectTypes = subSerializableObjectTypes;
        }
        #endregion
    }
}
