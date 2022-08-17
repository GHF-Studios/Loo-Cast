namespace LooCast.Data
{
    public interface ISaveable
    {
        void Save(bool saveDefault = false);
        void Load();
    }
}
