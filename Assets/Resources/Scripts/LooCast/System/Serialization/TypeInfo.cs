using System;

namespace LooCast.System.Serialization
{
    public abstract class TypeInfo
    {
        #region Properties
        public Type Type { get; private set; }
        public Serializability Serializability { get; private set; }
        #endregion

        #region Constructors
        public TypeInfo(Type type, Serializability serializability)
        {
            Type = type;
            Serializability = serializability;
        }
        #endregion
    }
}
