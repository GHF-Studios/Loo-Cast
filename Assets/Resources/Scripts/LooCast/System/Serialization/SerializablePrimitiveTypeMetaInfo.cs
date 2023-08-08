using System;

namespace LooCast.System.Serialization
{
    public class SerializablePrimitiveTypeMetaInfo : SerializableTypeMetaInfo
    {
        #region Constructors
        public SerializablePrimitiveTypeMetaInfo(Type serializableType) : base(SerializableTypeMetaInfoType.Primitive, serializableType, null, null, false, false)
        {
        }
        #endregion
    }
}
