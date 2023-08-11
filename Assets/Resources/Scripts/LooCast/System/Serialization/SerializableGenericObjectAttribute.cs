using System;

namespace LooCast.System.Serialization
{
    [AttributeUsage(AttributeTargets.Class, AllowMultiple = false)]
    public sealed class SerializableGenericObjectAttribute : Attribute
    {
        #region Constructors
        public SerializableGenericObjectAttribute()
        {
        }
        #endregion
    }
}
