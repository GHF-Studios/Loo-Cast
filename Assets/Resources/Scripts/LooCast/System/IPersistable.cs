namespace LooCast.System
{
    public interface IPersistable
    {
        #region Methods
        public void SaveRecursively();
        public void LoadRecursively();
        #endregion
    }
}
