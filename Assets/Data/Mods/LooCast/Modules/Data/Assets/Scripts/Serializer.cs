using System;

namespace LooCast.Data
{
    public abstract class Serializer
    {
        #region Properties
        public abstract string ID { get; }
        #endregion

        #region Methods
        public abstract void Serialize<T>(T data) where T : IData;
        public abstract T Deserialize<T>(string dataID) where T : IData;
        #endregion
    }
}
