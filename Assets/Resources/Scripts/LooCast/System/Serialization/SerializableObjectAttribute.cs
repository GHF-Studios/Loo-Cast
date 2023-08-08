using System;

namespace LooCast.System.Serialization
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public class SerializableObjectAttribute : SerializableAttribute
    {
        #region Constructors
        public SerializableObjectAttribute(bool overrideSerialization = false, bool overrideDeserialization = false, bool overrideSerializableTypeInfoCaching = false) : base(overrideSerialization, overrideDeserialization, overrideSerializableTypeInfoCaching)
        {
        }
        #endregion
    }
}
