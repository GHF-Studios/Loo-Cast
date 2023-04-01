namespace LooCast.System
{
    public class Identifier
    {
        #region Properties
        public string GUSID => gusid;
        #endregion

        #region Fields
        private string gusid;
        #endregion

        #region Constructors
        public Identifier(string gusid)
        {
            this.gusid = gusid;
        }
        #endregion
    }
}
