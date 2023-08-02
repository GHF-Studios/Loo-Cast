using System;

namespace LooCast.System.Serialization
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public class SerializableFolderAttribute : SerializableAttribute
    {
        #region Constructors
        public SerializableFolderAttribute(bool overrideSerialization = false, bool overrideDeserialization = false) : base(overrideSerialization, overrideDeserialization)
        {
        }
        #endregion
    }
}
