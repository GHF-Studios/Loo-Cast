using System;

namespace LooCast.System.Serialization
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public class SerializableFileAttribute : SerializableAttribute
    {
        #region Constructors
        public SerializableFileAttribute(bool overrideSerialization = false, bool overrideDeserialization = false) : base(overrideSerialization, overrideDeserialization)
        {
        }
        #endregion
    }
}
