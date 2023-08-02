using System;

namespace LooCast.System.Serialization
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public class SerializableObjectAttribute : SerializableAttribute
    {
        #region Constructors
        public SerializableObjectAttribute(bool overrideSerialization = false, bool overrideDeserialization = false) : base(overrideSerialization, overrideDeserialization)
        {
        }
        #endregion
    }
}
