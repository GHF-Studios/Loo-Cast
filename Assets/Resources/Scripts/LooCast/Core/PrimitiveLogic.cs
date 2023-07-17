namespace LooCast.Core
{
    public abstract class PrimitiveLogic
    {
        #region Properties
        public string LogicID { get; private set; }
        #endregion

        #region Constructors
        protected PrimitiveLogic(string logicID)
        {
            LogicID = logicID;
        }
        #endregion
    }
}
