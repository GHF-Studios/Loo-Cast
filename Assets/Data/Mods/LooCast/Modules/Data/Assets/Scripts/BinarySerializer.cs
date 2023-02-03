using System;

namespace LooCast.Data
{
    public class BinarySerializer : Serializer
    {
        #region Properties
        public override string ID
        {
            get
            {
                return "Binary";
            }
        }
        #endregion

        #region Methods
        public override T Deserialize<T>(string dataID)
        {
            
        }

        public override void Serialize<T>(T data)
        {

        }
        #endregion
    }
}
