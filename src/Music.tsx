import { FunctionalComponent } from "preact"

import { open, exists, BaseDirectory, readDir, DirEntry, stat } from '@tauri-apps/plugin-fs';
import { IAudioMetadata, parseBuffer, parseWebStream } from "music-metadata";
import { useSurrealClient } from "./context/components/SurrealProvider";
//import { useSurreal } from "./context/components/SurrealProvider";
// when using `"withGlobalTauri": true`, you may use
// const { exists, BaseDirectory } = window.__TAURI__.fs;

// Check if the `$APPDATA/avatar.png` file exists
//await exists('avatar.png', { baseDir: BaseDirectory.AppData });

type Artist = {
        id: string;
        name: string;
}

export const Music: FunctionalComponent = () => {

        const client = useSurrealClient();
        //const { client } = useSurreal();
        //console.log(client)

        const getTrackMetadataFromPath = async (path: string) => {
                const fs = await open(path, { read: true, baseDir: BaseDirectory.Audio });

                const stat = await fs.stat();
                //TODO: use a transform stream for this so we don't read the whole fucking file
                let buf = new Uint8Array(stat.size);
                let metadata: IAudioMetadata | undefined;
                await fs.read(buf);
                try {
                        metadata = await parseBuffer(buf, { size: stat.size, path: path });

                        let name = metadata.common.title || path.split('.').slice(0, -1).join('.')
                        name = ((metadata.common.album + '_') || '') + name;
                        name = ((metadata.common.artist + '_') || '') + name;
                        console.log(name);

                        //const song = gun.get('music').get('songs').get(name);
                        //song.put({ title: metadata.common.title || name })
                        //song.put({ path: path})
                        ////metadata.common.comment || song.put({comment: metadata.common.comment[0] || ''});
                        //if (metadata.common.lyrics !== undefined) {
                        //        const lyrics = metadata.common.lyrics[0];
                        //        if (lyrics.text !== undefined && lyrics != null)
                        //                song.get('lyrics').put(lyrics.text)
                        //        //for (const lyrics of metadata.common.lyrics) {
                        //        //}
                        //}
                        //if (metadata.common.year !== undefined) song.put({ year: metadata.common.year });


                        //let album = null;
                        //let artist = null;
                        //if (metadata.common.album !== undefined) {
                        //        const album_name = metadata.common.album.replaceAll('.', '_');
                        //        album = gun.get('music').get('albums').get(album_name);
                        //        song.put({ album: album });
                        //        const track_no = metadata.common.track.no?.toString() || '0';
                        //        album.get('songs').get(track_no).put(song);
                        //        album.get('title').put(metadata.common.album);
                        //}

                        //if (metadata.common.artist !== undefined) {
                        //        const artist_name = metadata.common.artist.replaceAll('.', '_');
                        //        artist = gun.get('music').get('artists').get(artist_name)
                        //        song.get('artist').put(artist);
                        //        artist.get('songs').set(song);
                        //        artist.get('name').put(metadata.common.artist);
                        //}

                        //if (artist !== undefined && artist != null && album !== undefined && album != null) {
                        //        //artist.get('albums').set(album)
                        //        //album.get('artist').put(artist)
                        //}

                } catch {
                }
                await fs.close();
                return metadata;
        }

        const scanDir = async function(dir: string) {
                const entries = await readDir(dir, { baseDir: BaseDirectory.Audio })
                for (const entry of entries) {
                        if (entry.isDirectory) {
                                //await scanDir(dir + entry.name + '/')
                                //await addRecordFromDir(dir + entry.name + '/')
                                return;
                        } else if (entry.isFile) {
                                //await addSongFromDir(dir + entry.name);
                        }
                }
        }

        const scan = async () => {
                await scanDir('')
        }

        const list = () => {
                //gun.get('music').get('albums').once().once((value, key) => {
                //gun.get('music').get('albums').once((value, key) => {
                //        console.log(key);
                //        console.log(value);
                //})
                //.map().once(function(value, key) {
                //        console.log(key)
                //        console.log(value)
                //});
        }

        return (
                <div>
                        <p> music </p>
                        <button onClick={scan} > scan </button>
                        <button onClick={list} > list </button>
                </div>
        )
}
