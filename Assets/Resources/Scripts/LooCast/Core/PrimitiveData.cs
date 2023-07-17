namespace LooCast.Core
{
    public abstract class PrimitiveData : IData
    {
        #region Properties
        public string DataID { get; private set; }
        #endregion

        #region Constructors
        protected PrimitiveData(string dataID)
        {
            DataID = dataID;
        }
        #endregion
    }
}
